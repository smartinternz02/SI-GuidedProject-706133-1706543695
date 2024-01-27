import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//WebUI.openBrowser('')

//WebUI.navigateToUrl('https://www.amazon.com/')

//WebUI.maximizeWindow()

//WebUI.click(findTestObject('Object Repository/Amazon_TL_SearchItem_Validation_OR/Page_Amazon.com. Spend less. Smile more/span_Hello, sign in'))

//WebUI.setText(findTestObject('Object Repository/Amazon_TL_SearchItem_Validation_OR/Page_Amazon Sign-In/input_email'), 'pulivarshit@gmail.com')

//WebUI.click(findTestObject('Object Repository/Amazon_TL_SearchItem_Validation_OR/Page_Amazon Sign-In/inputcontinue'))

//WebUI.setEncryptedText(findTestObject('Object Repository/Amazon_TL_SearchItem_Validation_OR/Page_Amazon Sign-In/input_password'), '8LAb5JbH9GYYbeHjJjwPWg==')

//WebUI.click(findTestObject('Object Repository/Amazon_TL_SearchItem_Validation_OR/Page_Amazon Sign-In/inputsignInSubmit'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Amazon_Search_Item_OR/Page_Amazon.com. Spend less. Smile more/select_All Departments        Arts  Crafts _135c92'), 
    'search-alias=stripbooks-intl-ship', true)

WebUI.setText(findTestObject('Object Repository/Amazon_Search_Item_OR/Page_Amazon.com. Spend less. Smile more/input_field-keywords'), 
    'Death Note')

WebUI.click(findTestObject('Object Repository/Amazon_Search_Item_OR/Page_Amazon.com. Spend less. Smile more/inputnav-search-submit-button'))

//WebUI.focus(findTestObject('Amazon_TL_SearchItem_Validation_OR/Page_Amazon.com. Spend less. Smile more/span_Account  Lists_nav-icon nav-arrow'))

//WebUI.click(findTestObject('Object Repository/Amazon_TL_SearchItem_Validation_OR/Page_Your Account/span_Sign Out'))

//WebUI.closeBrowser()

