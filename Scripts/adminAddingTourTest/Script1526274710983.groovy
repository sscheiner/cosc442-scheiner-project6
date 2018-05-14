import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.phptravels.net/admin')

WebUI.setText(findTestObject('Page_Administator Login (4)/input_email'), 'admin@phptravels.com')

WebUI.setText(findTestObject('Page_Administator Login (4)/input_password'), 'demoadmin')

WebUI.sendKeys(findTestObject('Page_Administator Login (4)/input_password'), Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Page_Dashboard (2)/a_Add New'))

WebUI.setText(findTestObject('Page_Add Tour/input_tourname'), 'test')

WebUI.click(findTestObject('Page_Add Tour/html_Rich Text Editor tourdesc'))

WebUI.setText(findTestObject('Page_Add Tour/input_maxadult'), '1')

WebUI.setText(findTestObject('Page_Add Tour/input_adultprice'), '10')

WebUI.setText(findTestObject('Page_Add Tour/input_tourdays'), '1')

WebUI.selectOptionByValue(findTestObject('Page_Add Tour/select_Select'), '1', true)

WebUI.setText(findTestObject('Page_Add Tour/input_tournights'), '1')

WebUI.click(findTestObject('Page_Add Tour/div_select2-drop-mask'))

WebUI.setText(findTestObject('Page_Add Tour/input_select2-input select2-fo'), 'washington')

WebUI.click(findTestObject('Page_Add Tour/button_Submit'))

WebUI.verifyElementPresent(findTestObject('Page_Tours Management/td_test'), 0)

WebUI.closeBrowser()

